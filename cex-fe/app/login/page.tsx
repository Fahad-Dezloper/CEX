"use client";
import axios from 'axios';
import { useRouter } from 'next/navigation';
import React, { useState } from 'react';
import { useUser } from '../context/UserContext';

const LoginUser = () => {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const router = useRouter();
    const { fetchUser } = useUser();

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        console.log(email, password, process.env.NEXT_PUBLIC_BASE_URL);
        const response = await axios.post(
            `${process.env.NEXT_PUBLIC_BASE_URL}/api/v1/auth/login`,
            { email, password },
            { withCredentials: true }
        );
        console.log(response);
        if (response.data.message == 'Login successful') {
            fetchUser();
            router.push('/');
        }
        return response.data;
    }
    
    return (
        <div className='flex flex-col items-center justify-center h-screen w-full'>
            <h1>Login User</h1>
            <form onSubmit={handleSubmit} className='flex flex-col gap-2 bg-white p-4 rounded-md'>
                <input type="email" value={email} onChange={(e) => setEmail(e.target.value)} placeholder='Email' className='border border-gray-300 rounded-md p-2 text-black' />
                <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} placeholder='Password' className='border border-gray-300 rounded-md p-2 text-black' />
                <button type="submit" className='bg-blue-500 text-white p-2 rounded-md'>Login</button>
            </form>
        </div>
    )
}

export default LoginUser;