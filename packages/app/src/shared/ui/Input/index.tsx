import { FC } from "react";

import styles from "./Input.module.scss";

interface InputProps {
  value: number | string;
  onChange: (event: React.ChangeEvent<HTMLInputElement>) => void;
  placeholder: string;
  type?: React.HTMLInputTypeAttribute;
  label?: string;
}

export const Input: FC<InputProps> = ({
  onChange,
  value,
  placeholder,
  label,
  type = "text",
}) => {
  return label ? (
    <label>
      {label}
      <input
        type={type}
        onChange={onChange}
        value={value}
        placeholder={placeholder}
        className={styles.input}
      />
    </label>
  ) : (
    <input
      type={type}
      onChange={onChange}
      placeholder={placeholder}
      value={value}
      className={styles.input}
    />
  );
};
