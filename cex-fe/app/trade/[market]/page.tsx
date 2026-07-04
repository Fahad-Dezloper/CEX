"use client";
import { MarketBar } from "@/app/components/MarketBar";
import { SwapUI } from "@/app/components/SwapUI";
import TradeTable from "@/app/components/TradeTable";
import { TradeView } from "@/app/components/TradeView";
import { Depth } from "@/app/components/depth/Depth";
import { useParams } from "next/navigation";

export default function Page() {
    const { market } = useParams();

    return <div className="flex px-6 gap-2">
        <div className="flex flex-col flex-1 overflow-hidden gap-2">
         <MarketBar market={market as string} />
             <div className="flex flex-row h-[600px] gap-2">
                 <div className="flex flex-col flex-1 rounded-lg overflow-hidden bg-[#14151B]">
                     <TradeView market={market as string} />
                 </div>
                 <div className="flex flex-col w-[300px] rounded-lg bg-[#14151B] overflow-hidden">
                     <Depth market={market as string} /> 
                 </div>
             </div>
             <div className="w-full h-fit p-4 primary-bg rounded-lg">
                <TradeTable />
             </div>
        </div>
        {/* <div className="w-px flex-col border-slate-800 border-l"></div> */}
        <div className="flex flex-col gap-2">
        <div className="primary-bg text-white p-4 h-fit rounded-lg">
            <div className="flex flex-col w-[300px] h-fit">
                <SwapUI market={market as string} />
            </div>
        </div>
        </div>

        {/* <div className="w-full h-[10vh] bg-red-300 absolute bottom-0"></div> */}
    </div>
}

// work for 100% not 90%