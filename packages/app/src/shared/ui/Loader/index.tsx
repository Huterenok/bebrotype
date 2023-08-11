import { FC } from "react";
import Image from "next/image";

import styles from "./Loader.module.scss";

export const Loader: FC = () => {
  return (
    <div className={styles.wrapper}>
      <Image
        width={400}
        height={400}
        alt="Loading..."
        src="/icons/banana.svg"
      />
    </div>
  );
};
