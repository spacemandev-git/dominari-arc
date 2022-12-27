import { FC, useRef, useContext } from "react";
import { GameContext, NavEnum } from "../pages/game";
import Settings from "./settings";
import Map from './map';

const GameBoard: FC = () => {
    const gameConext = useContext(GameContext);

    if (gameConext.nav == NavEnum.Settings) {
        return(<Settings></Settings>)
    } else if (gameConext.nav == NavEnum.GameBoard) {
        return (<Map></Map>)
    } else if (gameConext.nav == NavEnum.Hand) {
        return (<p>Hand</p>)
    } else {
        return(<></>)
    }
}
export default GameBoard;