"use client";

import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ArrowLeftRight, BitcoinIcon, DollarSign, DollarSignIcon } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";
import { Slider } from "@/components/ui/slider";
import { Separator } from "@/components/ui/separator";

type TradeSide = "buy" | "sell";
type OrderType = "limit" | "market";

export function SwapUI({ market }: { market: string }) {
  const [side, setSide] = useState<TradeSide>("buy");
  const [orderType, setOrderType] = useState<OrderType>("limit");

  return (
    <div className="flex flex-col h-fit ">
      <Tabs
        value={side}
        onValueChange={(value) => setSide(value as TradeSide)}
        className="flex flex-col gap-3"
      >
        <TabsList className="flex h-[50px] w-full gap-0 rounded-lg overflow-hidden bg-[#202127] p-0">
          <TabsTrigger
            value="buy"
            className={`flex h-full flex-1 flex-col justify-center rounded-xl text-[#75798a] hover:text-[#0DAF6E] text-base cursor-pointer font-semibold transition-none data-[state=active]:bg-[#222D2E] data-[state=active]:text-[#0DAF6E] `}
          >
            Buy
          </TabsTrigger>
          <TabsTrigger
            value="sell"
            className="flex h-full flex-1 flex-col justify-center rounded-xl  text-[#75798a] hover:text-[#FD4C4D] text-base cursor-pointer font-semibold  transition-none data-[state=active]:bg-[#39252A] data-[state=active]:text-[#FD4C4D] "
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
    <div className="flex flex-col gap-1 ">
      <Tabs
        value={orderType}
        onValueChange={(value) => onOrderTypeChange(value as OrderType)}
        className="flex flex-col"
      >
        <TabsList className="flex w-fit rounded-none bg-transparent p-0">
          <TabsTrigger
            value="limit"
            className="data-[state=active]:bg-[#1F2026] text-sm px-3 text-[#8991A0] data-[state=active]:text-white w-fit"
          >
            Limit
          </TabsTrigger>
          <TabsTrigger
            value="market"
            className="data-[state=active]:bg-[#1F2026] text-sm px-3 text-[#8991A0] data-[state=active]:text-white w-fit"
          >
            Market
          </TabsTrigger>
        </TabsList>
        <TabsContent value="limit" className="mt-0">
          <div className="flex flex-col mt-1.5">
            <div className="w-full text-xs flex accent justify-between">
              <span className="relative">
                <p>Balance</p>
                <span className="border-base-border-med border-[#37383C] absolute bottom-0 left-0 w-full translate-y-full border-b border-dashed"></span>
              </span>
              <span className="text-white">0 BTC</span>
            </div>
            <div className="flex flex-col gap-3 mt-4">
              {/* Price */}
              <div className="flex flex-col gap-1">
                <div className="flex items-center justify-between mb-1">
                  <span className="text-xs text-[#8991A0]">Price</span>
                  <div className="flex gap-2">
                    <span className="font-bold text-xs text-[#2586ed] cursor-pointer hover:underline">Mid</span>
                    <div className="w-[2px] h-4 bg-base-border-med rounded-full bg-[#37383C]" />
                    <span className="font-bold text-xs text-[#2586ed] cursor-pointer hover:underline">BBO</span>
                  </div>
                </div>
                <div className="relative">
                  <Input
                    type="number"
                    inputMode="decimal"
                    placeholder="0"
                    className="w-full h-fit rounded-lg py-1.5 px-4 bg-[#202127] border-none active:border-none active:outline-none focus-visible:ring-[#4B95FF] [&::-webkit-inner-spin-button]:appearance-none
    [&::-webkit-outer-spin-button]:m-0
    [appearance:textfield]"
                    style={{ fontSize: "1.6rem" }}
                  />
                  <span className="absolute right-3 top-1/2 -translate-y-1/2 flex items-center">
                    <span className="inline-flex items-center p-1 justify-center w-6 h-6 rounded-full green-bg">
                      <DollarSign className="font-bold text-white" />
                    </span>
                  </span>
                </div>
              </div>

              {/* Quantity */}
              <div className="flex flex-col gap-1">
                <Label className="block text-xs text-[#8991A0] mb-1">Quantity</Label>
                <div className="relative">
                  <Input
                    type="number"
                    inputMode="decimal"
                    placeholder="0"
                    className="w-full h-fit rounded-lg py-1.5  px-4 bg-[#202127] border-none active:border-none active:outline-none focus-visible:ring-[#4B95FF] [&::-webkit-inner-spin-button]:appearance-none
    [&::-webkit-outer-spin-button]:m-0
    [appearance:textfield]"
                    style={{ fontSize: "1.6rem" }}
                  />
                  <span className="absolute right-3 top-1/2 -translate-y-1/2 flex items-center">
                    <span className="inline-flex items-center justify-center w-7 h-7 rounded-full bg-[#FF9800]">
                      <BitcoinIcon className="font-bold text-white p-0.5" />
                    </span>
                  </span>
                </div>
                {/* Slider */}
                <div className=" pt-4.5 flex flex-col gap-2">
                  <div className="w-full flex items-center">
                    <Slider
                      value={[0]}
                      max={100}
                      min={0}
                      step={1}
                      disabled
                      className="w-full"
                      style={{ accentColor: "#3676da" }}
                    />
                  </div>
                  <div className="flex justify-between px-1.5 text-[#8991A0] text-[11px] font-medium">
                    <span>0</span>
                    <span>100%</span>
                  </div>
                </div>
              </div>

              {/* Order Value */}
              <div className="flex flex-col gap-1">
                <Label className="block text-xs text-[#8991A0] mb-1">Order Value</Label>
                <div className="relative">
                  <Input
                    type="number"
                    inputMode="decimal"
                    placeholder="0"
                    className="w-full h-fit rounded-lg py-1.5 px-4 bg-[#202127] border-none active:border-none active:outline-none focus-visible:ring-[#4B95FF] [&::-webkit-inner-spin-button]:appearance-none
    [&::-webkit-outer-spin-button]:m-0
    [appearance:textfield]"
                    style={{ fontSize: "1.6rem" }}
                  />
                  <span className="absolute right-3 top-1/2 -translate-y-1/2 flex items-center">
                    <span className="inline-flex items-center p-1 justify-center w-6 h-6 rounded-full green-bg">
                      <DollarSign className="font-bold text-white" />
                    </span>
                  </span>
                </div>
              </div>

               {/* Important Text */}
               <div className="flex flex-col w-full gap-2 text-xs">
                <div className="flex w-full items-center justify-between">
                  <span className="text-[#8991A0]">Borrow Amount(4.01%)</span>
                  <span className="text-white">0 USD</span>
                </div>

                <div className="flex w-full items-center justify-between"> 
                  <span className="text-[#8991A0]">Max Buying Amount</span>
                  <span className="text-white">0.00000 BTC</span>
                </div>
              </div>

              {/* CTA Button */}
              <Button
                type="button"
                className={`"w-full h-14 mt-2 rounded-xl text-lg cursor-pointer font-semibold ${ctaLabel == "Sell" ? "red-bg text-[#232323] hover:red-bg/80 " : "green-bg text-[#232323] hover:green-bg/80 "}  border-none focus-visible:ring-0 focus-visible:outline-none`}
              >
                {ctaLabel}
              </Button>
              {/* Checkboxes */}
              {/* <div className="flex gap-5 sm:gap-7 mt-2 items-center">
                <div className="flex items-center space-x-2">
                  <Checkbox id="postOnly" className="accent-[#404451] border-2 border-[#404451] data-[state=checked]:border-2 text-xl data-[state=checked]:border-[#404451] data-[state=checked]:bg-transparent" />
                  <Label htmlFor="postOnly" className="text-sm text-[#8991A0] cursor-pointer">
                    Post Only
                  </Label>
                </div>
                <div className="flex items-center space-x-2">
                  <Checkbox id="ioc" className="accent-[#404451] border-2 border-[#404451] data-[state=checked]:border-2 text-xl data-[state=checked]:border-[#404451] data-[state=checked]:bg-transparent" />
                  <Label htmlFor="ioc" className="text-sm text-[#8991A0] cursor-pointer">
                    IOC
                  </Label>
                </div>
                <div className="flex items-center space-x-2">
                  <Checkbox id="margin" className="accent-[#404451] border-2 border-[#404451] data-[state=checked]:border-2 text-xl data-[state=checked]:border-[#404451] data-[state=checked]:bg-transparent" />
                  <Label htmlFor="margin" className="text-sm text-[#8991A0] cursor-pointer">
                    Margin
                  </Label>
                </div>
              </div> */}
              <div className="flex gap-5 sm:gap-7 mt-2 items-center">
                <div className="flex items-center space-x-2">
                <Checkbox id="postOnly" className="accent-[#404451] border-2 border-[#404451] data-[state=checked]:border-2 text-xl data-[state=checked]:border-[#404451] data-[state=checked]:bg-transparent" />
                  <Label htmlFor="postOnly" className="text-sm text-[#8991A0] cursor-pointer">
                    Post Only
                  </Label>
                </div>
                <div className="flex items-center space-x-2">
                  <Checkbox id="ioc" className="accent-[#404451] border-2 border-[#404451] data-[state=checked]:border-2 text-xl data-[state=checked]:border-[#404451] data-[state=checked]:bg-transparent" />
                  <Label htmlFor="ioc" className="text-sm text-[#8991A0] cursor-pointer">
                    IOC
                  </Label>
                </div>
                <div className="flex items-center space-x-2">
                  <Checkbox id="margin" className="accent-[#404451] border-2 border-[#404451] data-[state=checked]:border-2 text-xl data-[state=checked]:border-[#404451] data-[state=checked]:bg-transparent" />
                  <Label htmlFor="margin" className="text-sm text-[#8991A0] cursor-pointer">
                    Margin
                  </Label>
                </div>
              </div>
            </div>
          </div>
        </TabsContent>
        <TabsContent value="market" className="mt-0">
        <div className="flex flex-col mt-2">
            <div className="w-full text-xs flex accent justify-between">
              <span className="relative">
                <p>Balance</p>
                <span className="border-base-border-med border-[#37383C] absolute bottom-0 left-0 w-full translate-y-full border-b border-dashed"></span>
              </span>
              <span className="text-white">0 BTC</span>
            </div>
            <div className="flex flex-col gap-4 mt-4">

              {/* Quantity */}
              <div className="flex flex-col gap-1">
                <div className="flex text-xs items-center justify-between mb-1">
                  <span className=" text-[#8991A0] flex items-center gap-1">Quantity <ArrowLeftRight className="w-4 h-4" /></span>
                  <div className="flex gap-2 accent">
                    ≈ 0.0 USD
                  </div>
                </div>
                <div className="relative">
                    
                  <Input
                    type="number"
                    inputMode="decimal"
                    placeholder="0"
                    className="w-full h-fit rounded-lg py-1.5  px-4 bg-[#202127] border-none active:border-none active:outline-none focus-visible:ring-[#4B95FF] [&::-webkit-inner-spin-button]:appearance-none
    [&::-webkit-outer-spin-button]:m-0
    [appearance:textfield]"
                    style={{ fontSize: "1.6rem" }}
                  />
                  <span className="absolute right-3 top-1/2 -translate-y-1/2 flex items-center">
                    <span className="inline-flex items-center justify-center w-7 h-7 rounded-full bg-[#FF9800]">
                      <BitcoinIcon />
                    </span>
                  </span>
                </div>
                {/* Slider */}
                <div className="pt-4.5 flex flex-col gap-2">
                  <div className="w-full flex items-center">
                    <Slider
                      value={[0]}
                      max={100}
                      min={0}
                      step={1}
                      disabled
                      className="w-full"
                      style={{ accentColor: "#3676da" }}
                    />
                  </div>
                  <div className="flex justify-between px-1.5 text-[#8991A0] text-[11px] font-medium">
                    <span>0</span>
                    <span>100%</span>
                  </div>
                </div>
              </div>


            {/* Important Text */}
            <div className="flex flex-col w-full gap-2 text-xs">
                <div className="flex w-full items-center justify-between">
                  <span className="text-[#8991A0]">Borrow Amount(4.01%)</span>
                  <span className="text-white">0 USD</span>
                </div>

                <div className="flex w-full items-center justify-between"> 
                  <span className="text-[#8991A0]">Max Buying Amount</span>
                  <span className="text-white">0.00000 BTC</span>
                </div>

                <div className="w-full text-xs flex accent justify-between">
              <span className="relative">
                <p>Max Slippage</p>
                <span className="border-base-border-med border-[#37383C] absolute bottom-0 left-0 w-full translate-y-full border-b border-dashed"></span>
              </span>
              <span className="text-[#4B95FF]">3%</span>
            </div>
              </div>

              {/* CTA Button */}
              <Button
                type="button"
                className={`"w-full h-14 mt-2 rounded-xl text-lg cursor-pointer font-semibold ${ctaLabel == "Sell" ? "red-bg text-[#232323] hover:red-bg/80 " : "green-bg text-[#232323] hover:green-bg/80 "}  border-none focus-visible:ring-0 focus-visible:outline-none`}
              >
                {ctaLabel}
              </Button>
              {/* Checkboxes */}
              <div className="flex gap-5 sm:gap-7 mt-2 items-center">
                <div className="flex items-center space-x-2">
                  <Checkbox id="margin" className="accent-[#404451] border-2 border-[#404451] data-[state=checked]:border-2 text-xl data-[state=checked]:border-[#404451] data-[state=checked]:bg-transparent" />
                  <Label htmlFor="margin" className="text-sm text-[#8991A0] cursor-pointer">
                    Margin
                  </Label>
                </div>
              </div>
            </div>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}