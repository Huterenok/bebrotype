"use client";

import { FC } from "react";

import { $user, Profile } from "enities/User";
import { UsersTexts } from "enities/Text";

import styles from "./ProfileInfo.module.scss";
import { useUnit } from "effector-react";

export const ProfileInfo: FC = () => {
  //TODO: Profile also have user store accessing
  const user = useUnit($user);

  return user ? (
    <div className={styles.wrapper}>
      <Profile />
      <UsersTexts id={user.id} />
    </div>
  ) : null;
  {
    /* TODO */
  }
};
