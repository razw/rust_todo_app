import { getTodos } from "@/lib/api";
import { TodoList } from "@/components/TodoList";

export default async function Home() {
  const todos = await getTodos();

  return (
    <main className="min-h-screen bg-gray-100 py-8">
      <div className="max-w-2xl mx-auto px-4">
        <h1 className="text-3xl font-bold text-center mb-8">
          TODO App
        </h1>
        <TodoList todos={todos} />
        </div>
    </main>
  );
}