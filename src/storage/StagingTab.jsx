import React, { useEffect, useState } from 'react';
import styled, { css } from "styled-components";
import DeleteIcon from '@mui/icons-material/Delete';
import CommitIcon from '@mui/icons-material/Commit';
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

const StagingTab = ({ reset, actions, setActions }) => {
    const [isPressed, setIsPressed] = useState(false);

    useEffect(() => {
        if (reset) {
            setActions([]);  // Clears the table, excluding the header
        }
    }, [reset]);

    const handleDelete = async (indexToDelete) => {
        setActions(currentActions => {
            const newActions = currentActions.filter((_, index) => index !== indexToDelete);
            commit(newActions);  // Call commit with the updated list of actions
            return newActions;
        });
    };

    const totalBytes = actions.reduce((acc, action) => acc + action.bytes, 0);

    const handleScanClick = async () => {
        setIsPressed(!isPressed);
        console.log('Commit button clicked!');
        try {
            const result = await commit(actions); // Pass the current actions to commit
            if (result !== undefined) {
                console.log("Scan successful, result:", result.toLocaleString());
            } else {
                console.log("Scan successful, but no data returned");
            }
        } catch (error) {
            console.error("Scan failed with error:", error);
        }
    };

    async function commit(actionsToSend) {
        try {
            let path = "foo";  // Path might be dynamically set based on your application's needs
            return await invoke('commit', { actions: actionsToSend });
        } catch (error) {
            console.error(error);
            throw error;
        }
    }

    const handleCommitEvent = (event) => {
        try {
            const { path } = JSON.parse(event.payload); // Assuming path is directly available in the event payload
            setActions(currentActions => currentActions.map(action =>
                action.path === path ? { ...action, status: 'X' } : action
            ));
        } catch (e) {
            console.error(`Error JSON encoded event ${event}, with error ${e}`);
        }
    };

    useEffect(() => {
        const unsubscribe = listen("commit-event", handleCommitEvent);

        // Cleanup the listener when the component unmounts
        return () => {
            unsubscribe.then((unsub) => unsub());
        };
    }, []);

    return (
        <div>
            <StagingHeaderContainer>
                <div className="flex-container">
                    <div className="flex-row">
                        <div className="flex-item">Bytes in scope</div>
                        <div className="flex-item">{totalBytes.toLocaleString("en-US")}</div>
                    </div>
                </div>
                <StagingCommit
                    as="button"
                    onClick={handleScanClick}
                    pressed={isPressed}
                >
                    <CommitIcon fontSize="large" style={{fontSize: '60px'}}/>
                </StagingCommit>
            </StagingHeaderContainer>
            <table>
                <thead>
                <tr>
                    <th style={{textAlign: "left"}}>Status</th>
                    <th style={{textAlign: "center"}}>Remove</th>
                    <th style={{textAlign: "left"}}>Action</th>
                    <th style={{textAlign: "left"}}>Resource</th>
                    <th style={{textAlign: "right"}}>Bytes</th>
                </tr>
                </thead>
                <tbody>
                {actions.map((actionObj, index) => (
                    <tr key={index}>
                        <td>{actionObj.status || 'Pending'}</td>
                        <td>
                            <DeleteIcon style={{padding: 0, textAlign: "center"}} onClick={() => handleDelete(index)} />
                        </td>
                        <td>{actionObj.action}</td>
                        <td>{actionObj.path}</td>
                        <td style={{textAlign: "right"}}>{actionObj.bytes.toLocaleString("en-US")}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
}

export default StagingTab;

const StagingHeaderContainer = styled.div`
    display: flex;
    line-height: 24px;
    font-weight: 400;
    color: #FFFFFF;
    overflow-y: hidden;
`;

const StagingCommit = styled.button`
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 4rem;
    margin-left: auto;
    margin-right: 30px;
    color: inherit;
    background: none;
    border: none;
    ${({pressed}) => pressed && css`
        color: #BBBBBB;
        transform: translateY(2px);
    `}
`;