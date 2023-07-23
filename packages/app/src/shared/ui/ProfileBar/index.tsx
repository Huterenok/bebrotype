import { FC } from "react";
import Image from "next/image";

import styles from "./ProfileBar.module.css";

interface ProfileBarProps {
  avatar: string;
  username: string;
}

export const ProfileBar: FC<ProfileBarProps> = ({ avatar, username}) => {
  return (
    <div className={styles.profile}>
      <Image
        alt="Profile avatar"
        src={avatar}
        width={32}
        height={32}
        className={styles.avatar}
      />
      {username.length > 12 ? `${username.slice(0, 12)}...` : username}
    </div>
  );
};
