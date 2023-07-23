import { FC } from "react";
import Link from "next/link";

import { ProfileBar } from "shared/ui/ProfileBar";

import styles from "./Header.module.css";

export const Header: FC = () => {
  return (
    <div className={styles.wrapper}>
      <div className={styles.container}>
        <Link href={"/profile"} className={styles.profile}>
          <ProfileBar username="Bebra" avatar="/icons/cat.jpg" />
        </Link>
      </div>
    </div>
  );
};
