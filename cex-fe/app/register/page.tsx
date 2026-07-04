"use client";
import axios from 'axios';
import React, { useState } from 'react';

const RegisterUser = () => {
    const [username, setUsername] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        console.log(username, email, password, process.env.NEXT_PUBLIC_BASE_URL);
        const response = await axios.post(`${process.env.NEXT_PUBLIC_BASE_URL}/api/v1/auth/register`, { username, email, password });
        console.log(response);
        return response.data;
    }
    
    return (
        <div className='flex flex-col items-center justify-center h-screen w-full'>
            <h1>Register User</h1>
            <form onSubmit={handleSubmit} className='flex flex-col gap-2 bg-white p-4 rounded-md'>
                <input type="text" value={username} onChange={(e) => setUsername(e.target.value)} placeholder='Username' className='border border-gray-300 rounded-md p-2 text-black' />
                <input type="email" value={email} onChange={(e) => setEmail(e.target.value)} placeholder='Email' className='border border-gray-300 rounded-md p-2 text-black' />
                <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} placeholder='Password' className='border border-gray-300 rounded-md p-2 text-black' />
                <button type="submit" className='bg-blue-500 text-white p-2 rounded-md'>Register</button>
            </form>
        </div>
    )
}

export default RegisterUser;