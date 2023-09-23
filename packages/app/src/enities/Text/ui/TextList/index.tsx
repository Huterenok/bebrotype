import { FC } from "react";

import { IText } from "../../types";

import { TextItem } from "./TextItem";
import styles from "./TextList.module.scss";

interface TextsListProps {
  texts: IText[];
}

export const TextList: FC<TextsListProps> = ({ texts }) => {
  //TODO: maybe fallback value?
  return (
    <div className={styles.wrapper}>
      {texts.map((text) => (
        <TextItem {...text} key={text.id} />
      ))}
    </div>
  );
};
