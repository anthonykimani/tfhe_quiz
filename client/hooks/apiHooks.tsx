import { useQuery } from "@tanstack/react-query";
import useAxios from "./useAxios";
import { AuthSuccess } from "@/types/form-types";

interface ApiHookResponse<T> {
  data: T | undefined;
  isLoading: boolean;
  error: unknown;
}

export function useGetLoginData(): ApiHookResponse<AuthSuccess> {
  const api = useAxios();
  const { isLoading, data, error } = useQuery({
    queryKey: ["getLoginData"],
    queryFn: () =>
      api.get(`auth/login`).then((res) => {
        return res.data;
      }),
  });
  return { isLoading, data, error };
}
