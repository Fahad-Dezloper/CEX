"use client";
import { useEffect, useState } from "react";
import { Ticker as TickerType } from "../utils/types";
import { getTicker } from "../utils/httpClient";

export const MarketBar = ({market}: {market: string}) => {
    const [ticker, setTicker] = useState<TickerType | null>(null);

    useEffect(() => {
        getTicker(market).then(setTicker);
    }, [market])

    return <div className="flex items-center p-2 flex-row relative w-full overflow-hidden border-b border-slate-800">
            <div className="flex items-center justify-between flex-row no-scrollbar overflow-auto pr-4">
                    <Ticker market={market} />
                    <div className="flex items-center flex-row space-x-8 pl-4">
                        <div className="flex h-full justify-center">
                            <p className={`font-medium tabular-nums text-greenText text-md text-green-500`}>${ticker?.lastPrice}1000</p>
                        </div>
                        <div className="flex flex-col">
                            <p className={`font-medium text-xs text-slate-400 text-sm`}>24H Change</p>
                            <p className={` text-sm font-medium tabular-nums leading-5 text-sm text-greenText ${Number(ticker?.priceChange) > 0 ? "text-green-500" : "text-red-500"}`}>{Number(ticker?.priceChange) > 0 ? "+" : ""} {ticker?.priceChange} {Number(ticker?.priceChangePercent)?.toFixed(2)}%</p></div><div className="flex flex-col">
                                <p className="font-medium text-xs text-slate-400 text-sm">24H High</p>
                                <p className="text-sm font-medium tabular-nums leading-5 text-sm ">{ticker?.high}</p>
                                </div>
                                <div className="flex flex-col">
                                    <p className="font-medium text-xs text-slate-400 text-sm">24H Low</p>
                                    <p className="text-sm font-medium tabular-nums leading-5 text-sm ">{ticker?.low}</p>
                                </div>
                            <button type="button" className="font-medium transition-opacity hover:opacity-80 hover:cursor-pointer text-base text-left" data-rac="">
                                <div className="flex flex-col">
                                    <p className="font-medium text-xs text-slate-400 text-sm">24H Volume</p>
                                    <p className="mt-1 text-sm font-medium tabular-nums leading-5 text-sm ">{ticker?.volume}
                                </p>
                            </div>
                        </button>
                    </div>
                </div>
            </div>

}

function Ticker({market}: {market: string}) {
    return <div className="flex items-center gap-2 py-2 px-4 text-white bg-zinc-600/20 rounded-lg hover:cursor-pointer hover:bg-zinc-600/30 transition-colors">
            <img alt="Base Asset Logo" loading="lazy" decoding="async" data-nimg="1" className="z-10 rounded-full h-6 w-6 outline-baseBackgroundL1"  src="/sol.webp" />
            <div className="text-white">{market.replace("-", " / ")}</div>
        </div>

}