'use server';

import { revalidatePath } from "next/cache";
import { createTodo } from "@/lib/api";

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
