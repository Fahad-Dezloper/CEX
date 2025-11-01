"use client";

import { usePathname } from "next/navigation";
import { PrimaryButton, SuccessButton } from "./core/Button"
import { useRouter } from "next/navigation";
import { useUser } from "../context/UserContext";
import Link from "next/link";

export const Appbar = () => {
    const router = useRouter();
    const route = usePathname();
    const { isLoggedIn, user, logout } = useUser();
    console.log(
        "user here too", user?.user?.assets.find((asset) => asset.symbol === "USDC")?.amount?.toString()
    );

    return <div className="text-white pt-4 pb-2 px-6 w-full">
        <div className="flex justify-between items-center">
            <div className="flex">
                <div className={`text-xl flex flex-col justify-center cursor-pointer text-white`} onClick={() => router.push('/')}>
                    FEX
                </div>
            </div>
            <div className="flex">
                {isLoggedIn ? (
                    <div className="">
                        <SuccessButton>
                            {`${user?.user?.assets.find((asset) => asset.symbol === "USDC")?.amount?.toString() ?? "0"} USDC`}
                        </SuccessButton>
                        <button onClick={logout} className="text-center font-semibold rounded-lg focus:ring-blue-200 focus:none focus:outline-none hover:opacity-90 disabled:opacity-80 disabled:hover:opacity-80 relative overflow-hidden h-[32px] text-sm px-3 py-1.5">
                            <div className="absolute inset-0 bg-blue-500 opacity-[16%]"></div>
                            <div className="flex flex-row items-center justify-center gap-4"><p className="text-blue-500">Logout</p></div>
                        </button>
                    </div>
                ) : (
                <div className="p-2 mr-2">
                    <Link href="/login">
                    <SuccessButton>Login</SuccessButton>
                    </Link>
                    <Link href="/register">
                        <PrimaryButton>Register</PrimaryButton>
                    </Link>
                </div>
                )}
            </div>
        </div>
    </div>
}