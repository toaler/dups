import React from 'react'
import styled from "styled-components"
import CommitIcon from '@mui/icons-material/Commit';

function StagingHeader({ totalBytes }) {
    const bytesReclaimed = 0.0;
    const filesDeleted = 0;
    const filesCompressed = 0;

    return <StagingHeaderContainer>
            <div className="flex-container">
                <div className="flex-row">
                    <div className="flex-item">Bytes in scope </div>
                    <div className="flex-item">{totalBytes.toLocaleString("en-US")}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Bytes reclaimed</div>
                    <div className="flex-item">{bytesReclaimed.toLocaleString("en-US")}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Files deleted</div>
                    <div className="flex-item">{filesDeleted.toLocaleString("en-US")}</div>
                </div>
                <div className="flex-row">
                    <div className="flex-item">Bytes compressed</div>
                    <div className="flex-item">{filesCompressed.toLocaleString("en-US")}</div>
                </div>
            </div>


        <StagingCommit>
            // TODO : add onclick
        </StagingCommit>


    </StagingHeaderContainer>
}

export default StagingHeader

const StagingHeaderContainer = styled.div`
    display: flex;
    line-height: 24px;
    font-weight: 400;
    color: #FFFFFF;
    overflow-y: hidden;
`;



const StagingCommit = styled(CommitIcon)`
    display: flex;
    font-size: 4rem !important; // Adjust the size as needed
    margin-left: auto;
    margin-right: 30px;
    color: inherit; // Or specify a color
`;