import { test, expect, APIRequestContext } from '@playwright/test';

const apiUrl = process.env.API_URL ?? 'http://localhost:3000';

async function cleanupTodos(request: APIRequestContext) {
  const res = await request.get(`${apiUrl}/todos`);
  if (!res.ok()) return;

  const todos = await res.json();
  for (const todo of todos) {
    await request.delete(`${apiUrl}/todos/${todo.id}`);
  }
}

test.describe.serial('Todo E2E', () => {
  test.beforeEach(async ({ page, request }) => {
    // テスト前にAPI経由で全件削除し、トップページへ遷移
    await cleanupTodos(request);
    await page.goto('/');
  });

  test('追加・完了・削除', async ({ page }) => {
    const title = `E2E ${Date.now()}`;

    // 追加フォームに入力して送信
    await page.getByPlaceholder('新しいTODOを入力...').fill(title);
    await page.getByRole('button', { name: '追加' }).click();

    // 追加された行が表示されるまで待機
    const row = page.getByText(title).locator('..');
    await expect(row).toBeVisible();

    // チェックを付けて完了状態へ
    const checkbox = row.locator('input[type="checkbox"]');
    await row.locator('label').click();
    await expect(checkbox).toBeChecked();
    await expect(page.getByText(title)).toHaveClass(/line-through/);

    // 削除確認ダイアログを許可して削除
    page.once('dialog', (dialog) => dialog.accept());
    await row.hover();
    await row.getByRole('button', { name: '削除' }).click();
    await expect(page.getByText(title)).toHaveCount(0);
  });

  test('並び替え', async ({ page }) => {
    const stamp = Date.now();
    const first = `E2E A ${stamp}`;
    const second = `E2E B ${stamp}`;

    // 2件追加して表示されるまで待機
    const input = page.getByPlaceholder('新しいTODOを入力...');
    await input.fill(first);
    await page.getByRole('button', { name: '追加' }).click();
    await expect(page.getByText(first)).toBeVisible({ timeout: 15000 });

    await input.fill(second);
    await page.getByRole('button', { name: '追加' }).click();
    await expect(page.getByText(second)).toBeVisible({ timeout: 15000 });

    // 対象行が描画されていることを確認
    const firstRow = page.locator('div.space-y-3 > div', { hasText: first });
    const secondRow = page.locator('div.space-y-3 > div', { hasText: second });
    await expect(firstRow).toBeVisible();
    await expect(secondRow).toBeVisible();

    // ドラッグハンドルで並び替え
    const dragHandle = firstRow.getByRole('button', { name: 'ドラッグして並び替え' });
    await dragHandle.dragTo(secondRow);

    // 表示順が入れ替わったことを確認
    const items = page.locator('div.space-y-3 > div');
    await expect(items.nth(0)).toContainText(second);
    await expect(items.nth(1)).toContainText(first);
  });
});
