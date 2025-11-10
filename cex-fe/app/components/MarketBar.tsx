"use client";
import { useEffect, useState } from "react";
import { Ticker as TickerType } from "../utils/types";
import { getTicker } from "../utils/httpClient";
import Image from "next/image";
import { ChevronDown } from "lucide-react";

export const MarketBar = ({market}: {market: string}) => {
    const [ticker, setTicker] = useState<TickerType | null>(null);

    useEffect(() => {
        getTicker(market).then(setTicker);
    }, [market])

    return <div className="flex items-center p-4 rounded-lg  primary-bg ">
            <div className="flex items-center gap-8 justify-between flex-row  whitespace-nowrap scroll-x pr-4">
                    {/* <Ticker market={market} /> */}
                    <div className="flex items-center w-fit px-2 py-3 text-white bg-zinc-600/20 rounded-lg hover:cursor-pointer hover:bg-zinc-600/30 transition-colors">
                        <div className="flex items-center gap-2">
                            <div className="w-[28px] h-[28px] flex items-center justify-center rounded-none">
                                <Image
                                    alt="Base Asset Logo"
                                    width={48}
                                    height={48}
                                    loading="lazy"
                                    decoding="async"
                                    src="/sol.webp"
                                    className="rounded-full object-cover"
                                />
                            </div>
                            <span className="text-white font-semibold  text-lg tracking-wide select-none" style={{ fontFamily: "inherit" }}>
                                {market.replace("-", " / ").toUpperCase()}
                            </span>

                            <ChevronDown className="accent" />
                        </div>
                    </div>
                    <div className="flex gap-6  items-center justify-center h-full text-white">
                        <div className="flex flex-col h-full justify-center ">
                            <p className={`font-medium tabular-nums text-greenText text-xl green-text`}>${ticker?.lastPrice}1000</p>
                            <p className={`font-medium tabular-nums text-greenText text-base text-white`}>${ticker?.lastPrice}1000.02</p>
                        </div>

                        <div className="flex flex-col h-full ">
                            <p className={`font-medium tabular-nums text-greenText text-xs accent`}>24H Change</p>
                            <p className={`font-medium tabular-nums text-greenText text-sm green-text`}>+582.8 +0.53%</p>
                        </div>

                        <div className="flex flex-col h-full ">
                            <p className={`font-medium tabular-nums text-greenText text-xs accent`}>24H High</p>
                            <p className={`font-medium tabular-nums text-greenText text-sm `}>111,077.2</p>
                        </div>

                        <div className="flex flex-col h-full ">
                            <p className={`font-medium tabular-nums text-greenText text-xs accent`}>24H Low</p>
                            <p className={`font-medium tabular-nums text-greenText text-sm `}>108,603.8</p>
                        </div>

                        {/* TODO: its clickable and changeble to btc - usd, usd - btc */}
                        <div className="flex flex-col h-full ">
                            <p className={`font-medium tabular-nums text-greenText text-xs accent`}>24H Volume (USD)</p>
                            <p className={`font-medium tabular-nums text-greenText text-sm `}>12,320,750.64</p>
                        </div>

                        <div className="flex flex-col h-full ">
                            <p className={`font-medium tabular-nums text-greenText text-xs accent`}>Lend APY (BTC / USD)</p>
                            <p className={`font-medium tabular-nums text-greenText text-sm green-text`}>0.01% / 5.56% ⚡️</p>
                        </div>

                        <div className="flex flex-col h-full ">
                            <p className={`font-medium tabular-nums text-greenText text-xs accent`}>Borrow APY (BTC / USD)</p>
                            <p className={`font-medium tabular-nums text-greenText text-sm red-text`}>0.22% / 4.31%</p>
                        </div>

                        <div className="flex flex-col h-full ">
                            <p className={`font-medium tabular-nums text-greenText text-xs accent`}>Interest Countdown</p>
                            <p className={`font-medium tabular-nums text-greenText text-sm `}>00:30:36</p>
                        </div>

                    </div>
                    
                </div>
            </div>

}

function Ticker({market}: {market: string}) {
    return 

}