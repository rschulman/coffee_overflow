import axios from "axios";
import type { LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UpdateHoursRequest, UpdateHoursResponse, UserDetailsResponse, UserResponse } from "./types";

const BASE_URL = "http://localhost:8080";
const api = axios.create({
  baseURL: BASE_URL,
  withCredentials: true, // if using a cookie
});

enum Endpoint {
    register = "register",
    login = "login",
    user = "user",
    userDetails = "user/details",
    userHours = "user/hours"
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

export const getUserDetails = async (): Promise<UserDetailsResponse> => {
    try {
        let res = await api.get<UserDetailsResponse>(urlString(Endpoint.userDetails));
        return res.data;
    } catch (error) {
        console.error('Get user details error:', error);
        throw error;
    }
};

export const updateHours = async (requestBody: UpdateHoursRequest): Promise<UpdateHoursResponse> => {
    try {
        let res = await api.post<UpdateHoursResponse>(urlString(Endpoint.userHours), requestBody);
        return res.data;
    } catch (error) {
        console.error('Update hours error:', error);
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