import axios from "axios";
import type { LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UserResponse } from "./types";

const BASE_URL = "http://localhost:8080";
const api = axios.create({
  baseURL: BASE_URL,
  withCredentials: true, // if using a cookie
});

enum Endpoint {
    register = "register",
    login = "login",
    user = "user"
}

const urlString = (path: Endpoint): string => {
    return `${BASE_URL}/${path.valueOf()}`;
}

export const register = async (requestBody: RegisterRequest): Promise<RegisterResponse> => {
    try {
        let res = await api.post<RegisterResponse>(urlString(Endpoint.register), requestBody);
        return res.data;
    } catch (error) {
        console.error('Registration error:', error);
        throw error;
    }
};

export const login = async (requestBody: LoginRequest): Promise<LoginResponse> => {
    try {
        let res = await api.post<LoginResponse>(urlString(Endpoint.login), requestBody);
        return res.data;
    } catch (error) {
        console.error('Login error:', error);
        throw error;
    }
};

export const getUser = async (): Promise<UserResponse> => {
    let res = await api.get<UserResponse>(Endpoint.user);
    return res.data;
};

export const updateUser = async (UserRequest): Promise<UserResponse> => {
    let res = await api.post<UserResponse>
};