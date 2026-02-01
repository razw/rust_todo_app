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
    <div className="group flex items-center gap-4 p-5 bg-gradient-to-r from-white to-gray-50 rounded-2xl shadow-md hover:shadow-xl transition-all duration-300 border border-gray-100">
      <label className="flex items-center cursor-pointer">
        <input
          type="checkbox"
          checked={todo.completed}
          onChange={handleToggle}
          readOnly
          className="sr-only peer"
        />
        <div className="relative w-6 h-6 bg-white border-2 border-gray-300 rounded-lg peer-checked:bg-gradient-to-br peer-checked:from-purple-500 peer-checked:to-pink-500 peer-checked:border-transparent transition-all duration-300 flex items-center justify-center">
          {todo.completed && (
            <svg className="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
            </svg>
          )}
        </div>
      </label>
      <span className={`flex-1 text-lg transition-all duration-300 ${todo.completed ? 'line-through text-gray-400' : 'text-gray-800'}`}>
        {todo.title}
      </span>
      <button
        onClick={handleDelete}
        className="p-2 text-gray-400 hover:text-red-500 transition-all duration-200 opacity-0 group-hover:opacity-100 hover:scale-110 transform"
        aria-label="削除"
      >
        <Trash2 size={22} />
      </button>
    </div>
  )
}
  