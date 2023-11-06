import { FC } from "react";

import styles from "./Keycup.module.scss";
import classNames from "classnames";

interface KeycupProps {
  value: string;
  isActive: boolean;
}

export const Keycup: FC<KeycupProps> = ({ value, isActive }) => {
  return (
    <div
      className={classNames(styles.wrapper, { [styles.isActive]: isActive })}
    >
      {value}
    </div>
  );
};
