import { Eye, EyeSlash } from "@phosphor-icons/react";
import { useField } from "formik";
import React, { useState } from "react";

const PasswordInput = ({ label, ...props }: any) => {
  const [field, meta] = useField(props);
  const [passwordVisibility, setPasswordVisibility] = useState("password");
  return (
    <div className="">
      <label
        htmlFor={props.id || props.name}
        className="block text-sm leading-6 text-gray-900"
      >
        {label}
      </label>
      <div className="flex justify-between rounded border border-gray-300 py-0 px-2 text-indigo-600 focus:ring-2 focus:ring-inset focus:ring-indigo-600 shadow-sm">
        <input
          type={passwordVisibility}
          {...field}
          {...props}
          className="border-none focus:border-none h-8 w-full sm:text-sm sm:leading-6 placeholder:text-gray-400 focus:ring-0"
        />
        <button
          type="button"
          onClick={() => {
            passwordVisibility == "password"
              ? setPasswordVisibility("text")
              : setPasswordVisibility("password");
          }}
        >
          {passwordVisibility == "password" ? (
            <EyeSlash size={24} />
          ) : (
            <Eye size={24} />
          )}
        </button>
      </div>
      {meta.touched && meta.error ? (
          <div className="error text-red-500 text-sm mt-1">{meta.error}</div>
        ) : null}
    </div>
  );
};

export default PasswordInput;
