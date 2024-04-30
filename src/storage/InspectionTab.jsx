import React, {useEffect, useState} from 'react';
import logger from "../logger.jsx";
import {listen} from "@tauri-apps/api/event";

import Tooltip from '@mui/material/Tooltip';
import FolderZipIcon from '@mui/icons-material/FolderZip';
import CompressIcon from '@mui/icons-material/Compress';
import DeleteIcon from '@mui/icons-material/Delete';
import TextSnippetIcon from '@mui/icons-material/TextSnippet';
import NotListedLocationIcon from '@mui/icons-material/NotListedLocation';
import ArchiveIcon from '@mui/icons-material/Archive';
import CodeIcon from '@mui/icons-material/Code';
import ImageIcon from '@mui/icons-material/Image';  // Ensure this icon is imported

function getMimeTypeIcon(compressible, mime_type) {
    if (compressible === "-1") {
        return (
            <Tooltip title={mime_type} placement="right">
                <FolderZipIcon/>
            </Tooltip>
        );
    }

    if (mime_type.startsWith('text/')) {
        return (
            <Tooltip title={mime_type} placement="right">
                <TextSnippetIcon/>
            </Tooltip>
        );
    }

    if (mime_type.startsWith('image/')) {
        return (
            <Tooltip title={mime_type} placement="right">
                <ImageIcon/>
            </Tooltip>
        );
    }

    switch (mime_type) {
        case 'application/octet-stream':
            return (
                <Tooltip title={mime_type} placement="right">
                    <NotListedLocationIcon/>
                </Tooltip>
            );
        case 'application/x-tar':
            return (
                <Tooltip title={mime_type} placement="right">
                    <ArchiveIcon/>
                </Tooltip>
            );
        default:
            if (mime_type.startsWith('application/')) {
                return (
                    <Tooltip title={mime_type} placement="right">
                        <CodeIcon/>
                    </Tooltip>
                );
            }
            return mime_type;
    }
}

function InspectionTab({reset, setActions}) {

    const [topKFiles, setTopKFiles] = useState([]);

    useEffect(() => {
        if (reset) {
            setTopKFiles([]);  // Clears the table, excluding the header
        }
    }, [reset]);

    useEffect(() => {
        const handleTopKEvent = (event) => {
            try {
                const data = JSON.parse(event.payload);
                setTopKFiles(data);
            } catch (e) {
                logger.error(`Error parsing JSON:`, e);
            }
        };

        const unsubscribe = listen("top-k-event", handleTopKEvent);

        return () => {
            unsubscribe.then((unsub) => unsub());
        };
    }, []);

    const handleIconClick = (event, action, path, bytes) => {
        event.stopPropagation();
        logger.info(`${action} action for path: ${path} with bytes: ${bytes}`);
        setActions(prevActions => {
            const existingIndex = prevActions.findIndex(actionObj => actionObj.path === path);
            if (existingIndex !== -1) {
                return prevActions.map((actionObj, index) =>
                    index === existingIndex ? {...actionObj, action, bytes} : actionObj
                );
            } else {
                return [...prevActions, {action, path, bytes}];
            }
        });
    };

    return (
        <div className="log-container">
            <link href="https://fonts.googleapis.com/css2?family=Roboto+Mono:wght@400&display=swap" rel="stylesheet"/>
            <table>
                <thead>
                <tr>
                    <th>Action</th>
                    <th>Rank</th>
                    <th>Bytes</th>
                    <th>Last Write</th>
                    <th>Last Read</th>
                    <th>Write Days</th>
                    <th>Read Days</th>
                    <th>Type</th>
                    <th>Path</th>
                </tr>
                </thead>
                <tbody>
                {topKFiles.map((row, index) => (
                    <tr key={index}>
                        <td>
                            <DeleteIcon onClick={(event) => handleIconClick(event, 'delete', row.path, Number(row.bytes))}/>
                            {row.compressible === "1" ? <CompressIcon onClick={(event) => handleIconClick(event, 'compress', row.path, Number(row.bytes))}/> : null}
                        </td>
                        <td>{row.rank}</td>
                        <td>{Number(row.bytes).toLocaleString("en-US")}</td>
                        <td>{row.modified}</td>
                        <td>{row.accessed}</td>
                        <td>{row.modified_days}</td>
                        <td>{row.accessed_days}</td>
                        <td>{getMimeTypeIcon(row.compressible, row.mime_type)}</td>
                        <td>{row.path}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
}

export default InspectionTab;
