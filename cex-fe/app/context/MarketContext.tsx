'use client'
import axios from 'axios'
import { useRouter } from 'next/navigation'
import { createContext, useContext, useState, useEffect } from 'react'

type MarketData = {
    base: string;
    max_order_size: number;
    max_price: number;
    min_order_size: number;
    min_price: number;
    price_precision: number;
    quantity_precision: number;
    quote: string;
    symbol: string;
}

const MarketContext = createContext({
    market: [] as MarketData[] | null,
})

export const MarketProvider = ({ children }: { children: React.ReactNode }) => {
    const [market, setMarket] = useState<MarketData[] | null>(null);

    // Move fetchUser outside so it can be reused elsewhere
    const fetchMarket = async () => {
        const response = await axios.get(`${process.env.NEXT_PUBLIC_BASE_URL}/api/v1/markets`, {
            withCredentials: true,
        });

        console.log("Market data", response.data);
        return response.data;
    };

    useEffect(() => {
        let cancelled = false;
        const getMarket = async () => {
            try {
                const userData = await fetchMarket();
                if (!cancelled) {
                    setMarket(userData);
                }
            } catch (error) {
                console.error(error);
            }
        };
        getMarket();
        return () => {
            cancelled = true;
        };
    }, []);

    return <MarketContext.Provider value={{ market }}>{children}</MarketContext.Provider>
}

export function useMarket() {
    const context = useContext(MarketContext);
    if (!context) {
        throw new Error('useMarket must be used within a MarketProvider');
    }
    return context;
}