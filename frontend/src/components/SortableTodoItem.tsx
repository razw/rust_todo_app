'use client';

import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { Todo } from "@/types/todo";
import { TodoItem } from "./TodoItem";
import { GripVertical } from "lucide-react";
import { useEffect, useState } from "react";

interface SortableTodoItemProps {
  todo: Todo;
}

export function SortableTodoItem({ todo }: SortableTodoItemProps) {
  const [isMounted, setIsMounted] = useState(false);

  useEffect(() => {
    queueMicrotask(() => setIsMounted(true));
  }, []);

  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: todo.id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.6 : 1,
    scale: isDragging ? '1.02' : '1',
  };

  if (!isMounted) {
    return null;
  }

  return (
    <div ref={setNodeRef} style={style} className="flex items-center gap-3 transition-all">
      <button
        {...attributes}
        {...listeners}
        className="p-2 text-gray-300 hover:text-purple-500 cursor-grab active:cursor-grabbing transition-colors rounded-lg hover:bg-purple-50"
        aria-label="ドラッグして並び替え"
      >
        <GripVertical size={22} />
      </button>
      <div className="flex-1">
        <TodoItem todo={todo} />
      </div>
    </div>
  );
}
