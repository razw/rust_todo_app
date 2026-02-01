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
    <form ref={formRef} action={handleSubmit} className="mb-8">
      <div className="flex gap-3">
        <input
          type="text"
          name="title"
          placeholder="新しいTODOを入力..."
          className="flex-1 px-6 py-4 border-2 border-gray-200 rounded-2xl focus:outline-none focus:border-purple-500 focus:ring-4 focus:ring-purple-100 transition-all text-lg"
          required
        />
        <button
          type="submit"
          className="px-8 py-4 bg-gradient-to-r from-purple-600 to-pink-600 text-white rounded-2xl hover:from-purple-700 hover:to-pink-700 transition-all font-semibold text-lg shadow-lg hover:shadow-xl hover:scale-105 transform"
        >
          追加
        </button>
      </div>
    </form>
  );
}
