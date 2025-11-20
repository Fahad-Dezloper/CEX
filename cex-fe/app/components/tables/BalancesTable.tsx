import {
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableFooter,
    TableHead,
    TableHeader,
    TableRow,
  } from "@/components/ui/table"
import { EllipsisVertical } from "lucide-react"
  
  const invoices = [
    {
      invoice: "INV002",
      paymentStatus: "Pending",
      totalAmount: "$150.00",
      paymentMethod: "PayPal",
    },
    {
      invoice: "INV002",
      paymentStatus: "Pending",
      totalAmount: "$150.00",
      paymentMethod: "PayPal",
    },
    {
      invoice: "INV003",
      paymentStatus: "Unpaid",
      totalAmount: "$350.00",
      paymentMethod: "Bank Transfer",
    },
    {
      invoice: "INV004",
      paymentStatus: "Paid",
      totalAmount: "$450.00",
      paymentMethod: "Credit Card",
    },
    {
      invoice: "INV005",
      paymentStatus: "Paid",
      totalAmount: "$550.00",
      paymentMethod: "PayPal",
    },
    {
      invoice: "INV006",
      paymentStatus: "Pending",
      totalAmount: "$200.00",
      paymentMethod: "Bank Transfer",
    },
    {
      invoice: "INV007",
      paymentStatus: "Unpaid",
      totalAmount: "$300.00",
      paymentMethod: "Credit Card",
    },
  ]
  
  export function BalancesTable() {
    return (
      <Table className="text-[#8991A0] border-none">
        <TableHeader className="">
          <TableRow className="hover:bg-transparent text-xs! border-b-[#323336]">
            <TableHead className="w-[100px] text-[#8991A0]">Asset</TableHead>
            <TableHead className="text-[#8991A0] text-right">Total Balance</TableHead>
            <TableHead className="text-[#8991A0] text-right">Available Balance</TableHead>
            <TableHead className="text-right text-[#8991A0]">Open Orders</TableHead>
            <TableHead className="text-right text-[#8991A0]"></TableHead>
          </TableRow>
        </TableHeader>
        <TableBody className="">
          {invoices.map((invoice) => (
            <TableRow key={invoice.invoice} className="tabular-nums border-b-[#323336] hover:bg-[#202127]">
              <TableCell className="font-medium">
                <div className="flex items-center gap-3">
                <img src="https://backpack.exchange/_next/image?url=%2Fcoins%2Fsol.png&w=96&q=95" alt="solana-img" className="w-8 h-8 border rounded-full border-[#323336]" />
                <div className="flex flex-col gap-1">
                  <span className="text-white">Solana</span>
                  <span className="text-xs">SOL</span>
                </div>
                </div>
              </TableCell>


              <TableCell className="text-right ">
                <div className="flex flex-col font-medium  gap-1">
                <span className="text-white">0.12020648</span>
                <span className="text-xs">$17.22</span>
                </div>
              </TableCell>


              <TableCell className="text-right">
              <div className="flex flex-col font-medium  gap-1">
                <span className="text-white">0.12020648</span>
                <span className="text-xs">$17.22</span>
                </div>
              </TableCell>
              <TableCell className="text-right text-white">0</TableCell>
              <TableCell className="flex text-right w-full items-center justify-end gap-6 text-[#4c94ff] font-semibold">
                  <span>Deposit</span>
                  <span>Withdraw</span>
                  <span><EllipsisVertical /></span>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    )
  }
  