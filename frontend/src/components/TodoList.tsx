'use client';

import { useState, useEffect } from 'react';
import {
  DndContext,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  DragEndEvent,
} from '@dnd-kit/core';
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
} from '@dnd-kit/sortable';
import { Todo } from "@/types/todo";
import { SortableTodoItem } from './SortableTodoItem';
import { reorderTodosAction } from '@/actions/todo';

interface TodoListProps {
  todos: Todo[];
}

export function TodoList({ todos: initialTodos }: TodoListProps) {
  const [todos, setTodos] = useState(initialTodos);

  // propsが変わったらstateを同期
  useEffect(() => {
    setTodos(initialTodos);
  }, [initialTodos]);

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  async function handleDragEnd(event: DragEndEvent) {
    const { active, over } = event;

    if (over && active.id !== over.id) {
      const oldIndex = todos.findIndex((t) => t.id === active.id);
      const newIndex = todos.findIndex((t) => t.id === over.id);

      const newTodos = arrayMove(todos, oldIndex, newIndex);
      setTodos(newTodos);

      // APIに並び順を保存
      const ids = newTodos.map((t) => t.id);
      const result = await reorderTodosAction(ids);
      if (result.error) {
        // エラー時は元に戻す
        setTodos(todos);
        alert(result.error);
      }
    }
  }

  if (todos.length === 0) {
    return (
      <div className="text-center py-12">
        <div className="text-6xl mb-4">📝</div>
        <p className="text-gray-500 text-lg">
          TODOがありません
        </p>
        <p className="text-gray-400 text-sm mt-2">
          上のフォームから新しいタスクを追加しましょう
        </p>
      </div>
    );
  }

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={closestCenter}
      onDragEnd={handleDragEnd}
    >
      <SortableContext items={todos.map((t) => t.id)} strategy={verticalListSortingStrategy}>
        <div className="space-y-3">
          {todos.map((todo) => (
            <SortableTodoItem key={todo.id} todo={todo} />
          ))}
        </div>
      </SortableContext>
    </DndContext>
  );
}
