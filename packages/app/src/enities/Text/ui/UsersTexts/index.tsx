"use client";
import { useUnit } from "effector-react";
import { FC, useEffect } from "react";

import { $userTexts, getUserTextsFn } from "../../model";
import { TextCard } from "../TextCard";

import styles from "./UsersTexts.module.scss";

interface UsersTextsProps {
  id: number;
}

//TODO - place of this component in file hierarchy
export const UsersTexts: FC<UsersTextsProps> = ({ id }) => {
  const [usersTexts, getUserTexts] = useUnit([$userTexts, getUserTextsFn]);

  useEffect(() => {
    getUserTexts(id);
  }, [id, getUserTexts]);

  return (
    <div className={styles.wrapper}>
      {usersTexts.length > 0
        ? usersTexts.map((text) => <TextCard {...text} key={text.id} />)
        : "You don't have any texts("}
    </div>
  );
};
