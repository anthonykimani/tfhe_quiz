// Define the expected type for the useAuth hook's return value
export interface AuthContextType {
  user: any; // Replace `any` with your actual user type
  login: (userData: any) => void; // Adjust according to the actual parameters and types
  logout: () => void;
}

export interface LoginFormFields {
  email: string;
  password: string;
}

// Define the types for the form data
export type SignUpFormData = {
  email: string;
  password: string;
  confirmPassword: string;
};

export type AuthSuccess = {
  token: string;
  user_reference: string;
  email: string;
  username: string;
};

export type ResultType = {
  score: number;
  correctAnswers: number;
  wrongAnswers: number;
};
