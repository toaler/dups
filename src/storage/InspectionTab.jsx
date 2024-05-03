import React, {useEffect, useState} from 'react';
import "./InspectionTab.css";
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
import ImageIcon from '@mui/icons-material/Image';

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
        <div className="inspect-container">
            <table>
                <thead>
                <tr>
                    <th className="center-text">Action</th>
                    <th className="center-text">Rank</th>
                    <th className="right-text">Bytes</th>
                    <th className="center-text">Last Write</th>
                    <th className="center-text">Last Read</th>
                    <th className="center-text">Write Days</th>
                    <th className="center-text">Read Days</th>
                    <th className="center-text">Type</th>
                    <th className="left-text">Path</th>
                </tr>
                </thead>
                <tbody>
                {topKFiles.map((row, index) => (
                    <tr key={index}>
                        <td className="center-text">
                            <DeleteIcon
                                onClick={(event) => handleIconClick(event, 'delete', row.path, Number(row.bytes))}
                            />
                            <CompressIcon
                                style={{color: row.compressible === "1" ? '#83f52c' : 'inherit'}}
                                onClick={(event) => handleIconClick(event, 'compress', row.path, Number(row.bytes))}
                            />
                        </td>
                        <td className="center-text">{row.rank}</td>
                        <td className="right-text">{Number(row.bytes).toLocaleString("en-US")}</td>
                        <td className="center-text">{row.modified}</td>
                        <td className="center-text">{row.accessed}</td>
                        <td className="center-text">{row.modified_days}</td>
                        <td className="center-text">{row.accessed_days}</td>
                        <td className="center-text">{getMimeTypeIcon(row.compressible, row.mime_type)}</td>
                        <td className="left-text">{row.path}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
}

export default InspectionTab;
