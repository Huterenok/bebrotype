import { FC } from "react";

import styles from "./Bar.module.scss";

interface BarProps {
	children: React.ReactNode
}

export const Bar: FC<BarProps> = ({children}) => {
  return (
    <div className={styles.bar}>
			{children}
    </div>
  );
};
