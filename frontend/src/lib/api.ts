import { Todo } from '@/types/todo';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

export async function getTodos(): Promise<Todo[]> {
  const response = await fetch(`${API_URL}/todos`);
  if (!response.ok) {
    throw new Error('Failed to fetch todos');
  }
  return response.json();
}

export async function createTodo(title: string): Promise<Todo> {
  const response = await fetch(`${API_URL}/todos`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ title }),
  });
  if (!response.ok) {
    throw new Error('Failed to create todo')
  }

  return response.json();
}
