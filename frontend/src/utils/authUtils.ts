import { defaultRequestInit } from "./fetchUtils.ts";

export const signIn = async (
  username: string,
  password: string,
  setAuth: (auth: boolean) => void
): Promise<void> => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const response = await fetch(`${VITE_BASE_API_URL}/auth/login`, {
    ...defaultRequestInit,
    method: "POST",
    body: JSON.stringify({ username, password }),
  });
  setAuth(response.ok);

  if (!response.ok) throw new Error("Failed on sign up request");
};

export const signOut = async (
  setAuth: (auth: boolean) => void
): Promise<void> => {
  const { VITE_BASE_API_URL } = import.meta.env;

  const response = await fetch(`${VITE_BASE_API_URL}/auth/logout`, {
    ...defaultRequestInit,
    method: "POST",
  });
  setAuth(!response.ok);

  if (!response.ok) throw new Error("Failed on sign out request");
};
