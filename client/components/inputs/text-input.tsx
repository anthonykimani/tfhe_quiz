import { useField } from "formik";
import React from "react";

const TextInput = ({ label, ...props }: any) => {
    const [field, meta] = useField(props);
  return (
    <div>
      <label
        htmlFor={props.id || props.name}
        className="block text-sm font-medium leading-6 text-gray-900"
      >
        {label}
      </label>
      <div className="mt-2">
        <input
          {...field}
          {...props}
          className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
        />
        {meta.touched && meta.error ? (
          <div className="error text-red-500 text-sm mt-1">{meta.error}</div>
        ) : null}
      </div>
    </div>
  );
};

export default TextInput;
