import { FC, useContext } from "react";
import { FaGamepad, FaWrench } from 'react-icons/fa';
import { CgCardSpades } from 'react-icons/cg';
import { GameContext, NavEnum } from "../pages/game";


const Menu: FC = () => {
    const gameConext = useContext(GameContext);

    return(
    <div className="fixed top-0 left-0 h-screen w-16 flex flex-col
                  bg-white dark:bg-gray-900 shadow-lg gap-4 items-center">
        <div className="sidebar-icon group mt-36">
            <FaWrench size="48" onClick={() => {gameConext.changeNav(NavEnum.Settings)}} />
        </div>
        <div className="sidebar-icon group">
            <FaGamepad size="48" onClick={() => {gameConext.changeNav(NavEnum.GameBoard)}} />
        </div>
        <div className="sidebar-icon group">
            <CgCardSpades size="48" onClick={() => {gameConext.changeNav(NavEnum.Hand)}} />
        </div>
    </div>
    )
}

export default Menu;
