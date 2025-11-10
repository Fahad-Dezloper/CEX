import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import react from "react";

const TradeTable = () => {
  return (
    <div className="w-full h-full flex flex-col">
      <Tabs defaultValue="Balances">
        <TabsList className="flex w-fit rounded-none bg-transparent p-0">
          <TabsTrigger
            value="Balances"
            className="data-[state=active]:bg-[#1F2026] text-sm px-3 text-[#8991A0] data-[state=active]:text-white w-fit"
          >
            Balances
          </TabsTrigger>
          <TabsTrigger
            value="Positions"
            className="data-[state=active]:bg-[#1F2026] text-sm px-3 text-[#8991A0] data-[state=active]:text-white w-fit"
          >
            Positions
          </TabsTrigger>
          <TabsTrigger
            value="Borrows"
            className="data-[state=active]:bg-[#1F2026] text-sm px-3 text-[#8991A0] data-[state=active]:text-white w-fit"
          >
            Borrows
          </TabsTrigger>
          <TabsTrigger
            value="Open Orders"
            className="data-[state=active]:bg-[#1F2026] text-sm px-3 text-[#8991A0] data-[state=active]:text-white w-fit"
          >
            Open Orders
          </TabsTrigger>
          <TabsTrigger
            value="TWAP"
            className="data-[state=active]:bg-[#1F2026] text-sm px-3 text-[#8991A0] data-[state=active]:text-white w-fit"
          >
            TWAP
          </TabsTrigger>
          <TabsTrigger
            value="Fill History"
            className="data-[state=active]:bg-[#1F2026] text-sm px-3 text-[#8991A0] data-[state=active]:text-white w-fit"
          >
            Fill History
          </TabsTrigger>
          <TabsTrigger
            value="Order History"
            className="data-[state=active]:bg-[#1F2026] text-sm px-3 text-[#8991A0] data-[state=active]:text-white w-fit"
          >
            Order History
          </TabsTrigger>
          <TabsTrigger
            value="Position History"
            className="data-[state=active]:bg-[#1F2026] text-sm px-3 text-[#8991A0] data-[state=active]:text-white w-fit"
          >
            Position History
          </TabsTrigger>
        </TabsList>
        <div className="w-full h-full py-2">
          <TabsContent value="Balances" className="w-full h-full flex flex-col">
            <div className="flex flex-col">
              <span className="primary-gray text-xs font-semibold">
                Your Balances
              </span>
              <div className="flex w-full justify-between items-center">
                <div className="flex gap-10 items-center">
                  <span className="text-lg text-white">$0.00</span>
                  <div className="w-fit h-fit bg-[#1F2026] text-[#8991A0] rounded-md px-2 py-1 flex items-center justify-center">
                    <span className="text-xs">$0 0.0%</span>
                  </div>
                </div>
                <div className="flex items-center space-x-2">
                <Checkbox id="zero" className="accent-[#404451] border-2 border-[#404451] data-[state=checked]:border-2 text-xl data-[state=checked]:border-[#404451] data-[state=checked]:bg-transparent" />
                  <Label htmlFor="zero" className="text-sm text-[#8991A0] cursor-pointer">
                        Hide zero balances
                  </Label>
                </div>
              </div>

              
            </div>
          </TabsContent>
        </div>
      </Tabs>
    </div>
  );
};
export default TradeTable;
