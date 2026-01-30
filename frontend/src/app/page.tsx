import { getTodos } from "@/lib/api";
import { TodoList } from "@/components/TodoList";
import { TodoForm } from "@/components/TodoForm";

// API に依存するためビルド時プリレンダをスキップ（CI で API が無いため）
export const dynamic = "force-dynamic";

export default async function Home() {
  const todos = await getTodos();

  return (
    <main className="min-h-screen bg-gray-100 py-8">
      <div className="max-w-2xl mx-auto px-4">
        <h1 className="text-3xl font-bold text-center mb-8">
          TODO App
        </h1>
        <TodoForm />
        <TodoList todos={todos} />
        </div>
    </main>
  );
}