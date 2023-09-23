"use client";

import { FC } from "react";

import { useUnit } from "effector-react";

import { UserTexts } from "widgets/UserTexts";
import { Profile } from "enities/Profile";
import { $user } from "enities/User";

import styles from "./ProfilePage.module.scss";

interface ProfilePageProps {
  id: number | null;
}

export const ProfilePage: FC<ProfilePageProps> = ({ id }) => {
  //TODO
  const user = useUnit($user);
  const exactId = id || user?.id!;
  const isOwner = !Boolean(id);

  return (
    <div className={styles.wrapper}>
      <Profile id={exactId} currentUser={isOwner ? user : null} />
      <UserTexts id={exactId} />
    </div>
  );
};
