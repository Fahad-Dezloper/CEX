import { TradesTable } from "./components/home/Trades";

export default function Home() {
  return (
    <main className="flex min-h-screen max-w-6xl mx-auto flex-col gap-8 items-center justify-between py-8">
      <div className="flex justify-between gap-8 w-full">
        <div className="h-[35vh] w-full bg-gray-200 rounded-xl"></div>
        <div className="h-[35vh] w-full bg-blue-200 rounded-xl"></div>
        <div className="h-[35vh] w-full bg-red-200 rounded-xl"></div>
      </div>
      <div className="w-full">
      <TradesTable />
      </div>
    </main>
  );
}
