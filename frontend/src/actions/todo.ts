'use server';

import { revalidatePath } from "next/cache";
import { createTodo } from "@/lib/api";
import { updateTodo } from "@/lib/api";

export async function addTodoAction(formData: FormData) {
  const title = formData.get('title') as string;

  if (!title || title.trim() === '') {
    return { error: 'タイトルを入力してください' };
  }

  try {
    await createTodo(title.trim());
    revalidatePath('/');
    return { success: true };
  } catch(error) {
    return { error: 'TODOの作成に失敗しました' }
  }
}

export async function toggleTodoAction(id: number, completed: boolean) {
  try {
    await updateTodo(id, { completed });
    revalidatePath('/');
    return { success: true };
  } catch(error) {
    return { error: 'TODOの更新に失敗しました' };
  }
}
