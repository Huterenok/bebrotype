import { FC } from "react";

import { AuthButton, AuthModal } from "features/Auth/ui";

import styles from "./Header.module.scss";

export const Header: FC = () => {
  return (
    <header className={styles.wrapper}>
      <div className={styles.container}>
        <AuthButton />
      </div>
      <AuthModal />
    </header>
  );
};
