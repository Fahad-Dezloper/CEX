"use client"

import { useState } from "react"
import { Card } from "@/components/ui/card"
import { TrendingUp, TrendingDown } from "lucide-react"
import { useMarket } from "@/app/context/MarketContext"
import Link from "next/link"


function MiniChart({ data, isPositive }: { data: number[]; isPositive: boolean }) {
  const max = Math.max(...data)
  const min = Math.min(...data)
  const range = max - min

  const points = data
    .map((value, index) => {
      const x = (index / (data.length - 1)) * 100
      const y = 100 - ((value - min) / range) * 100
      return `${x},${y}`
    })
    .join(" ")

  return (
    <svg width="120" height="40" className="overflow-visible">
      <polyline
        points={points}
        fill="none"
        stroke={isPositive ? "#10b981" : "#ef4444"}
        strokeWidth="2"
        vectorEffect="non-scaling-stroke"
      />
    </svg>
  )
}

export function TradesTable() {
  const [activeTab, setActiveTab] = useState("spot");
  const { market } = useMarket();

  return (
    <Card className="bg-transparent border-zinc-800 p-4" >
      <div className="flex items-center">
        {["spot", "futures"].map((tab) => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className={`text-sm font-medium capitalize transition-colors px-4 py-2 relative ${
              activeTab === tab ? "text-white bg-zinc-600/20 rounded-lg" : "text-zinc-500 hover:text-white"
            }`}
          >
            {tab}
          </button>
        ))}
      </div>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="border-b border-zinc-800">
              <th className="text-left py-3 px-2 text-xs font-medium text-zinc-500 uppercase tracking-wider">Name</th>
              <th className="text-right py-3 px-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">Price</th>
              <th className="text-right py-3 px-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">
                24h Volume
              </th>
              <th className="text-right py-3 px-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">
                Open Interest
              </th>
              <th className="text-right py-3 px-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">
                24h Change
              </th>
              <th className="text-right py-3 px-4 text-xs font-medium text-zinc-500 uppercase tracking-wider">
                Last 7 Days
              </th>
            </tr>
          </thead>
          <tbody>
            {activeTab == "spot" ? 
            <>
            {market?.map((item, index) => {
              return (
                <tr
                  key={item.symbol}
                  className={`${index == market?.length - 1 ? "" : "border-b border-zinc-800"} hover:bg-zinc-800/30 transition-colors cursor-pointer`}
                  onClick={() => window.location.href = `/trade/${item.symbol}`}
                  style={{ cursor: "pointer" }}
                >
                  <td className="py-4 px-2">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 rounded-full bg-zinc-800 flex items-center justify-center text-xl">
                        {item.symbol.charAt(0)}
                      </div>
                      <span className="font-semibold text-white">{item.base}</span>
                    </div>
                  </td>
                  <td className="py-4 px-4 text-right font-mono font-medium text-white">{item.price_precision}</td>
                  <td className="py-4 px-4 text-right font-mono text-zinc-400">{item.quantity_precision}</td>
                  <td className="py-4 px-4 text-right font-mono text-zinc-400">{item.max_order_size}</td>
                  <td className="py-4 px-4 text-right">
                    <span
                      className={`inline-flex items-center gap-1 font-medium ${
                        item.max_price > item.min_price ? "text-emerald-500" : "text-red-500"
                      }`}
                    >
                      {item.max_price > item.min_price ? <TrendingUp className="w-4 h-4" /> : <TrendingDown className="w-4 h-4" />}
                      {item.max_price > item.min_price ? "+" : ""}
                      {item.max_price}
                    </span>
                  </td>
                  <td className="py-4 px-4 text-right">
                    <div className="flex justify-end">
                      <MiniChart data={item.max_price > item.min_price ? [item.max_price, item.min_price] : [item.min_price, item.max_price]} isPositive={item.max_price > item.min_price} />
                    </div>
                  </td>
                </tr>
              )
            })}
            </>
            :
            <div className="flex h-40 items-center justify-center text-xl uppercase text-white font-semibold">its coming soon</div>
            }
          </tbody>
        </table>
      </div>
    </Card>
  )
}
