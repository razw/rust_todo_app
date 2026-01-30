'use client';

import { Todo } from "@/types/todo";
import { toggleTodoAction } from "@/actions/todo";
import { deleteTodoAction } from "@/actions/todo";
import { Trash2 } from 'lucide-react';
  
interface TodoItemProps {
  todo: Todo;
}

export function TodoItem({ todo }: TodoItemProps) {
  async function handleToggle() {
    const result = await toggleTodoAction(todo.id, !todo.completed);
    if (result.error) {
      alert(result.error);
    }
  }

  async function handleDelete() {
    if (!confirm('このTODOを削除しますか？')) {
      return;
    }
    const result = await deleteTodoAction(todo.id);
    if (result.error) {
      alert(result.error);
    }
  }

  return (
    <div className="flex items-center gap-3 p-4 bg-white rounded-lg shadow">
      <input
        type="checkbox"
        checked={todo.completed}
        onChange={handleToggle}
        readOnly
        className="w-5 h-5"
      />
      <span className={`flex-1 ${todo.completed ? 'line-through text-gray-400' : ''}`}>
        {todo.title}
      </span>
      <button
        onClick={handleDelete}
        className="p-2 text-gray-400 hover:text-red-500 transition-colors"
        aria-label="削除"
      >
        <Trash2 size={20} />
      </button>
    </div>
  )
}
  