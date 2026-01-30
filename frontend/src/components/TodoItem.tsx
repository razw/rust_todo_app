'use client';

import { Todo } from "@/types/todo";
import { toggleTodoAction } from "@/actions/todo";
  
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

  return (
    <div className="flex items-center gap-3 p-4 bg-white rounded-lg shadow">
      <input
        type="checkbox"
        checked={todo.completed}
        onChange={handleToggle}
        readOnly
        className="w-5 h-5"
      />
      <span className={todo.completed ? 'line-through text-gray-400' : ''}>
        {todo.title}
      </span>
    </div>
  )
}
  