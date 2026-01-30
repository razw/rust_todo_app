'use client';

import { useRef } from "react";
import { addTodoAction } from "@/actions/todo";

export function TodoForm() {
  const formRef = useRef<HTMLFormElement>(null);

  async function handleSubmit(formData: FormData) {
    const result = await addTodoAction(formData);
    if (result.success) {
      formRef.current?.reset();
    } else if (result.error) {
      alert(result.error);
    }
  }

  return (
    <form ref={formRef} action={handleSubmit} className="mb-6">
      <div className="flex gap-2">
        <input
          type="text"
          name="title"
          placeholder="新しいTODOを入力..."
          className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          required
        />
        <button
          type="submit"
          className="px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
        >
          追加
        </button>
      </div>
    </form>
  );
}
