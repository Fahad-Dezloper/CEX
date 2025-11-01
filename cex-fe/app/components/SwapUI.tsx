"use client";

import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

type TradeSide = "buy" | "sell";
type OrderType = "limit" | "market";

export function SwapUI({ market }: { market: string }) {
  const [side, setSide] = useState<TradeSide>("buy");
  const [orderType, setOrderType] = useState<OrderType>("limit");

  return (
    <div className="flex flex-col">
      <Tabs
        value={side}
        onValueChange={(value) => setSide(value as TradeSide)}
        className="flex flex-col gap-1"
      >
        <TabsList className="flex h-[55px] w-full gap-0 rounded-lg bg-[#202127] p-0">
          <TabsTrigger
            value="buy"
            className={`flex h-full flex-1 flex-col justify-center rounded-lg text-[#75798a] hover:text-[#0DAF6E] text-base cursor-pointer font-semibold transition-none data-[state=active]:bg-[#222D2E] data-[state=active]:text-[#0DAF6E] `}
          >
            Buy
          </TabsTrigger>
          <TabsTrigger
            value="sell"
            className="flex h-full flex-1 flex-col justify-center rounded-lg  text-[#75798a] hover:text-[#FD4C4D] text-base cursor-pointer font-semibold  transition-none data-[state=active]:bg-[#39252A] data-[state=active]:text-[#FD4C4D] "
          >
            Sell
          </TabsTrigger>
        </TabsList>

        <TabsContent value="buy" className="mt-0">
          <OrderForm
            side="buy"
            orderType={orderType}
            onOrderTypeChange={setOrderType}
          />
        </TabsContent>
        <TabsContent value="sell" className="mt-0">
          <OrderForm
            side="sell"
            orderType={orderType}
            onOrderTypeChange={setOrderType}
          />
        </TabsContent>
      </Tabs>
    </div>
  );
}

type OrderFormProps = {
  side: TradeSide;
  orderType: OrderType;
  onOrderTypeChange: (type: OrderType) => void;
};

function OrderForm({ side, orderType, onOrderTypeChange }: OrderFormProps) {
  const ctaLabel = side === "buy" ? "Buy" : "Sell";

  return (
    <div className="flex flex-col gap-1">
      <div className="px-3">
        <Tabs
          value={orderType}
          onValueChange={(value) => onOrderTypeChange(value as OrderType)}
          className="flex flex-col"
        >
          <TabsList className="flex w-fit  gap-5 rounded-none bg-transparent p-0">
            <TabsTrigger
              value="limit"
              className="data-[state=active]:bg-[#1F2026] text-[#8991A0] data-[state=active]:text-white w-fit"
            >
              Limit
            </TabsTrigger>
            <TabsTrigger
              value="market"
              className="data-[state=active]:bg-[#1F2026] w-fit"
            >
              Market
            </TabsTrigger>
          </TabsList>
        </Tabs>
      </div>
      <div className="flex flex-col px-3">
        <div className="flex flex-col flex-1 gap-3 text-baseTextHighEmphasis">
          <div className="flex flex-col gap-3">
            <div className="flex flex-row items-center justify-between">
              <p className="text-xs font-normal text-baseTextMedEmphasis">Available Balance</p>
              <p className="text-xs font-medium text-baseTextHighEmphasis">36.94 USDC</p>
            </div>
          </div>
          <div className="flex flex-col gap-2">
            <p className="text-xs font-normal text-baseTextMedEmphasis">Price</p>
            <div className="relative flex flex-col">
              <input
                step="0.01"
                placeholder="0"
                className="h-12 rounded-lg border-2 border-solid border-baseBorderLight bg-background pr-12 text-right text-2xl leading-9 text-[$text] placeholder-baseTextMedEmphasis ring-0 transition focus:border-accentBlue focus:ring-0"
                type="text"
                value="134.38"
              />
              <div className="absolute right-1 top-1 flex flex-row p-2">
                <div className="relative">
                  <img src="/usdc.webp" className="h-6 w-6" />
                </div>
              </div>
            </div>
          </div>
        </div>
        <div className="flex flex-col gap-2">
          <p className="text-xs font-normal text-baseTextMedEmphasis">Quantity</p>
          <div className="relative flex flex-col">
            <input
              step="0.01"
              placeholder="0"
              className="h-12 rounded-lg border-2 border-solid border-baseBorderLight bg-background pr-12 text-right text-2xl leading-9 text-[$text] placeholder-baseTextMedEmphasis ring-0 transition focus:border-accentBlue focus:ring-0"
              type="text"
              value="123"
            />
            <div className="absolute right-1 top-1 flex flex-row p-2">
              <div className="relative">
                <img src="/sol.webp" className="h-6 w-6" />
              </div>
            </div>
          </div>
          <div className="flex flex-row justify-end">
            <p className="pr-2 text-xs font-medium text-baseTextMedEmphasis">≈ 0.00 USDC</p>
          </div>
          <div className="mt-2 flex flex-row justify-center gap-3">
            <div className="flex cursor-pointer flex-row items-center justify-center rounded-full bg-baseBackgroundL2 px-[16px] py-[6px] text-xs hover:bg-baseBackgroundL3">
              25%
            </div>
            <div className="flex cursor-pointer flex-row items-center justify-center rounded-full bg-baseBackgroundL2 px-[16px] py-[6px] text-xs hover:bg-baseBackgroundL3">
              50%
            </div>
            <div className="flex cursor-pointer flex-row items-center justify-center rounded-full bg-baseBackgroundL2 px-[16px] py-[6px] text-xs hover:bg-baseBackgroundL3">
              75%
            </div>
            <div className="flex cursor-pointer flex-row items-center justify-center rounded-full bg-baseBackgroundL2 px-[16px] py-[6px] text-xs hover:bg-baseBackgroundL3">
              Max
            </div>
          </div>
        </div>
        <Button
          type="button"
          className="my-4 h-12 rounded-xl bg-greenPrimaryButtonBackground px-4 py-2 text-base font-semibold text-greenPrimaryButtonText focus-visible:outline-none focus-visible:ring-0 active:scale-98"
        >
          {ctaLabel}
        </Button>
        <div className="mt-1 flex flex-row justify-between">
          <div className="flex flex-row gap-2">
            <div className="flex items-center">
              <input
                className="form-checkbox h-5 w-5 cursor-pointer rounded border border-solid border-baseBorderMed bg-base-950 font-light text-transparent shadow-none outline-none ring-0 ring-transparent checked:border-baseBorderMed checked:bg-base-900 checked:hover:border-baseBorderMed focus:bg-base-900 focus:ring-0 focus:ring-offset-0 focus:checked:border-baseBorderMed"
                id="postOnly"
                type="checkbox"
                data-rac=""
              />
              <label className="ml-2 text-xs" htmlFor="postOnly">
                Post Only
              </label>
            </div>
            <div className="flex items-center">
              <input
                className="form-checkbox h-5 w-5 cursor-pointer rounded border border-solid border-baseBorderMed bg-base-950 font-light text-transparent shadow-none outline-none ring-0 ring-transparent checked:border-baseBorderMed checked:bg-base-900 checked:hover:border-baseBorderMed focus:bg-base-900 focus:ring-0 focus:ring-offset-0 focus:checked:border-baseBorderMed"
                id="ioc"
                type="checkbox"
                data-rac=""
              />
              <label className="ml-2 text-xs" htmlFor="ioc">
                IOC
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}