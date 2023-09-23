"use client";

import { FC, useEffect } from "react";
import { useUnit } from "effector-react";

import {
  $error,
  $isPending,
  $userTexts,
  getUserTextsEv,
} from "../../UserTexts/model";

import { TextList } from "enities/Text";

import styles from "./UserTexts.module.scss";

interface ProfileInfoProps {
  id: number;
}

export const UserTexts: FC<ProfileInfoProps> = ({ id }) => {
  const [texts, error, isPending, getUserTexts] = useUnit([
    $userTexts,
    $error,
    $isPending,
    getUserTextsEv,
  ]);

  useEffect(() => {
    getUserTexts(id);
  }, [id]);

  //TODO
  if (error) {
    return <div>Something went wrong</div>;
  }

  //TODO
  return isPending ? (
    <div>Loading...</div>
  ) : (
    <div className={styles.wrapper}>
      <TextList texts={texts} />
    </div>
  );
};
