"use client";
import { MarketBar } from "@/app/components/MarketBar";
import { SwapUI } from "@/app/components/SwapUI";
import { TradeView } from "@/app/components/TradeView";
import { Depth } from "@/app/components/depth/Depth";
import { useParams } from "next/navigation";

export default function Page() {
    const { market } = useParams();

    return <div className="flex px-6 gap-2">
        <div className="flex flex-col flex-1 overflow-hidden gap-2">
            {/* <div className="h-[30vh]"></div> */}
         <MarketBar market={market as string} />
             <div className="flex flex-row h-[600px] gap-2">
                 <div className="flex flex-col flex-1 rounded-lg overflow-hidden bg-blue-300">
                     <TradeView market={market as string} />
                 </div>
                 <div className="flex flex-col w-[300px] rounded-lg bg-red-300 overflow-hidden">
                     <Depth market={market as string} /> 
                 </div>
             </div>
        </div>
        {/* <div className="w-px flex-col border-slate-800 border-l"></div> */}
        <div className="primary-bg text-white p-4 rounded-lg">
            <div className="flex flex-col w-[350px]">
                <SwapUI market={market as string} />
            </div>
        </div>
    </div>
}