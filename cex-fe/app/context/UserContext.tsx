'use client'
import axios from 'axios'
import { useRouter } from 'next/navigation'
import { createContext, useContext, useState, useEffect } from 'react'

interface UserAsset {
    symbol: string;
    amount: number;
}

interface ActualUser {
    id: string;
    email: string;
    username: string;
    assets: UserAsset[];
}

interface User {
    user: ActualUser | null;
}



const UserContext = createContext({
    user: null as User | null,
    isLoggedIn: false,
    setUser: (user: any) => {},
    setIsLoggedIn: (isLoggedIn: boolean) => {},
    logout: () => {},
    fetchUser: async () => {},
})

export const UserProvider = ({ children }: { children: React.ReactNode }) => {
    const [user, setUser] = useState<User | null>(null);
    const [isLoggedIn, setIsLoggedIn] = useState(false);
    const router = useRouter();
    
    const logout = async () => {
        await axios.post(`${process.env.NEXT_PUBLIC_BASE_URL}/api/v1/auth/logout`, {
            withCredentials: true,
        });
        setIsLoggedIn(false);
        router.push('/login');
    }
    // Move fetchUser outside so it can be reused elsewhere
    const fetchUser = async () => {
        const response = await axios.get(`${process.env.NEXT_PUBLIC_BASE_URL}/api/v1/auth/me`, {
            withCredentials: true,
        });
        return response.data;
    };

    useEffect(() => {
        let cancelled = false;
        const getUser = async () => {
            try {
                const userData = await fetchUser();
                if (!cancelled) {
                    setUser(userData);
                    setIsLoggedIn(true);
                }
            } catch (error) {
                if (!cancelled) {
                    setIsLoggedIn(false);
                    router.push('/login');
                }
            }
        };
        getUser();
        return () => {
            cancelled = true;
        };
    }, []);

    return <UserContext.Provider value={{ user, isLoggedIn, setUser, setIsLoggedIn, logout, fetchUser }}>{children}</UserContext.Provider>
}

export function useUser() {
    const context = useContext(UserContext);
    if (!context) {
        throw new Error('useUser must be used within a UserProvider');
    }
    return context;
}