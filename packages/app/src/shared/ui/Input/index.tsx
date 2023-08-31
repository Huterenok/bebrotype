import { FC, InputHTMLAttributes } from "react";

import styles from "./Input.module.scss";

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: string;
}

export const Input: FC<InputProps> = ({
  onChange,
  value,
  placeholder,
  label,
  name,
  type = "text",
}) => {
  return label ? (
    <div>
      <label htmlFor={name} className={styles.label}>
        {label}
      </label>
      <input
        type={type}
        onChange={onChange}
        value={value}
        name={name}
        placeholder={placeholder}
        className={styles.input}
        //TODO
        id={name}
      />
    </div>
  ) : (
    <input
      type={type}
      onChange={onChange}
      value={value}
      name={name}
      placeholder={placeholder}
      className={styles.input}
    />
  );
};
