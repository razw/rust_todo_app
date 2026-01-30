  import { Todo } from "@/types/todo"
  
interface TodoItemProps {
  todo: Todo;
}

export function TodoItem({ todo }: TodoItemProps) {
  return (
    <div className="flex items-center gap-3 p-4 bg-white rounded-lg shadow">
      <input
        type="checkbox"
        checked={todo.completed}
        readOnly
        className="w-5 h-5"
      />
      <span className={todo.completed ? 'line-through text-gray-400' : ''}>
        {todo.title}
      </span>
    </div>
  )
}
  