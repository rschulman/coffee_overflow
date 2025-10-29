import axios from "axios";
import type { LoginRequest, LoginResponse, UserResponse } from "./types";

// TODO: What should this be?
const BASE_URL = "https://localhost:8080";
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

export const register = async (requestBody: RegisterRequest) => {
    let res = await api.post<RegisterResponse>(Endpoint.register, requestBody);
    return res.data;
};

export const login = async (requestBody: LoginRequest) => {
    try {
        let res = await api.post<LoginResponse>(urlString(Endpoint.login), requestBody);
    } catch (error) {
        // TODO: update state with error message for user?
        console.log(error);
    }
    // TODO: httpOnly cookie?
    // sessionStorage.setItem('token', res.data.token);
};

export const getUser = async (): Promise<UserResponse> => {
    let res = await api.get<UserResponse>(Endpoint.user);
    return res.data;
};

export const updateUser = async (UserRequest): Promise<UserResponse> => {
    let res = await api.post<UserResponse>
};