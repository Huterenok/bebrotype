import { FC } from "react";

import { Profile } from "enities/User";

import styles from "./ProfileInfo.module.scss";

export const ProfileInfo: FC = () => {
  return (
    <div className={styles.wrapper}>
      <Profile />
    </div>
  );
};
