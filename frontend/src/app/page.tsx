import { getTodos } from "@/lib/api";
import { TodoList } from "@/components/TodoList";
import { TodoForm } from "@/components/TodoForm";

// API に依存するためビルド時プリレンダをスキップ（CI で API が無いため）
export const dynamic = "force-dynamic";

export default async function Home() {
  const todos = await getTodos();

  return (
    <main className="min-h-screen bg-gradient-to-br from-purple-500 via-pink-500 to-orange-400 py-12">
      <div className="max-w-3xl mx-auto px-4">
        <div className="text-center mb-10">
          <h1 className="text-6xl font-bold text-white mb-2 drop-shadow-lg">
            TODO App
          </h1>
          <p className="text-white/90 text-lg">あなたのタスクを整理しよう</p>
        </div>
        <div className="bg-white/95 backdrop-blur-sm rounded-3xl shadow-2xl p-8">
          <TodoForm />
          <TodoList todos={todos} />
        </div>
      </div>
    </main>
  );
}